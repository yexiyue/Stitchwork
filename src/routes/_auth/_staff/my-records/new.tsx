import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { NavBar, Toast, Form, Button, Picker, Stepper } from "antd-mobile";
import type { Order, Process } from "@/types";
import { pieceRecordApi, orderApi, processApi } from "@/api";
import { useQueryClient, useMutation, useQuery } from "@tanstack/react-query";
import { useState } from "react";
import { useAuthStore } from "@/stores/auth";
import { OrderPicker } from "@/components";

export const Route = createFileRoute("/_auth/_staff/my-records/new")({
  component: StaffNewRecordPage,
});

function StaffNewRecordPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const user = useAuthStore((s) => s.user);

  const [orderId, setOrderId] = useState("");
  const [processId, setProcessId] = useState("");
  const [quantity, setQuantity] = useState(1);
  const [orderSheetVisible, setOrderSheetVisible] = useState(false);

  const { data: ordersData } = useQuery({
    queryKey: ["orders-active"],
    queryFn: () => orderApi.list({ pageSize: 1000, status: ["pending", "processing"] }),
  });

  const { data: processesData } = useQuery({
    queryKey: ["processes", orderId],
    queryFn: () => processApi.list({ orderId, pageSize: 1000 }),
    enabled: !!orderId,
  });

  const createMutation = useMutation({
    mutationFn: pieceRecordApi.create,
    onSuccess: () => {
      Toast.show({ content: "已提交，待老板审核" });
      queryClient.invalidateQueries({ queryKey: ["piece-records"] });
      navigate({ to: "/my-records" });
    },
  });

  const processOptions = (processesData?.list ?? []).map((p: Process) => ({
    label: `${p.name} (¥${p.piecePrice})`,
    value: p.id,
  }));

  const selectedOrder = (ordersData?.list ?? []).find((o) => o.id === orderId);
  const selectedProcess = (processesData?.list ?? []).find(
    (p) => p.id === processId
  );

  const totalAmount = selectedProcess
    ? (parseFloat(selectedProcess.piecePrice) * quantity).toFixed(2)
    : "0.00";

  const handleSelectOrder = () => {
    setOrderSheetVisible(true);
  };

  const handleOrderSelect = (order: Order) => {
    setOrderId(order.id);
    setProcessId("");
    setOrderSheetVisible(false);
  };

  const handleSelectProcess = async () => {
    if (!orderId) {
      Toast.show({ content: "请先选择订单" });
      return;
    }
    const val = await Picker.prompt({ columns: [processOptions] });
    if (val) setProcessId(val[0] as string);
  };

  const handleSubmit = () => {
    if (!processId || quantity < 1 || !user?.id) {
      Toast.show({ content: "请填写完整信息" });
      return;
    }
    createMutation.mutate({
      userId: user.id,
      processId,
      quantity,
    });
  };

  return (
    <div>
      <NavBar onBack={() => navigate({ to: "/my-records" })}>
        录入计件
      </NavBar>
      <div className="p-4">
        <Form
          footer={
            <Button
              block
              color="primary"
              onClick={handleSubmit}
              loading={createMutation.isPending}
            >
              提交
            </Button>
          }
        >
          <Form.Item
            label="订单"
            required
            clickable
            onClick={handleSelectOrder}
            extra={selectedOrder?.productName || "请选择订单"}
          />
          <Form.Item
            label="工序"
            required
            clickable
            onClick={handleSelectProcess}
            extra={
              selectedProcess
                ? `${selectedProcess.name} (¥${selectedProcess.piecePrice})`
                : orderId
                ? "请选择工序"
                : "请先选择订单"
            }
          />
          <Form.Item label="数量" required>
            <Stepper min={1} value={quantity} onChange={setQuantity} />
          </Form.Item>
          <Form.Item label="金额">
            <span className="text-lg font-medium text-orange-500">
              ¥{totalAmount}
            </span>
          </Form.Item>
        </Form>
        <div className="text-center text-gray-400 text-sm mt-4">
          提交后需等待老板审核
        </div>
      </div>
      <OrderPicker
        visible={orderSheetVisible}
        orders={ordersData?.list ?? []}
        onSelect={handleOrderSelect}
        onClose={() => setOrderSheetVisible(false)}
      />
    </div>
  );
}
