import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { NavBar, Toast, Form, Button, Picker, Stepper } from "antd-mobile";
import type { Staff, Order, Process } from "@/types";
import { pieceRecordApi, orderApi, processApi } from "@/api";
import { useQueryClient, useMutation, useQuery } from "@tanstack/react-query";
import { useState } from "react";
import { useStaffList } from "@/hooks";

export const Route = createFileRoute("/_auth/_boss/records/new")({
  component: NewRecordPage,
});

function NewRecordPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const [userId, setUserId] = useState("");
  const [orderId, setOrderId] = useState("");
  const [processId, setProcessId] = useState("");
  const [quantity, setQuantity] = useState(1);

  const { data: staffList } = useStaffList();
  const { data: ordersData } = useQuery({
    queryKey: ["orders-all"],
    queryFn: () => orderApi.list({ pageSize: 1000 }),
  });
  const { data: processesData } = useQuery({
    queryKey: ["processes", orderId],
    queryFn: () => processApi.list({ orderId, pageSize: 1000 }),
    enabled: !!orderId,
  });

  const createMutation = useMutation({
    mutationFn: pieceRecordApi.create,
    onSuccess: () => {
      Toast.show({ content: "创建成功" });
      queryClient.invalidateQueries({ queryKey: ["piece-records"] });
      navigate({ to: "/records" });
    },
  });

  const staffOptions = (staffList?.list ?? []).map((s: Staff) => ({
    label: s.displayName || s.username,
    value: s.id,
  }));

  const orderOptions = (ordersData?.list ?? []).map((o: Order) => ({
    label: o.productName,
    value: o.id,
  }));

  const processOptions = (processesData?.list ?? []).map((p: Process) => ({
    label: `${p.name} (¥${p.piecePrice})`,
    value: p.id,
  }));

  const selectedStaff = staffOptions.find((o) => o.value === userId);
  const selectedOrder = orderOptions.find((o) => o.value === orderId);
  const selectedProcess = (processesData?.list ?? []).find((p) => p.id === processId);

  const totalAmount = selectedProcess ? (parseFloat(selectedProcess.piecePrice) * quantity).toFixed(2) : "0.00";

  const handleSelectStaff = async () => {
    const val = await Picker.prompt({ columns: [staffOptions] });
    if (val) setUserId(val[0] as string);
  };

  const handleSelectOrder = async () => {
    const val = await Picker.prompt({ columns: [orderOptions] });
    if (val) {
      setOrderId(val[0] as string);
      setProcessId("");
    }
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
    if (!userId || !processId || quantity < 1) {
      Toast.show({ content: "请填写完整信息" });
      return;
    }
    createMutation.mutate({ userId, processId, quantity });
  };

  return (
    <div>
      <NavBar onBack={() => navigate({ to: "/records" })}>新增计件</NavBar>
      <div className="p-4">
        <Form
          footer={
            <Button
              block
              color="primary"
              onClick={handleSubmit}
              loading={createMutation.isPending}
            >
              保存
            </Button>
          }
        >
          <Form.Item
            label="员工"
            required
            clickable
            onClick={handleSelectStaff}
            extra={selectedStaff?.label || "请选择员工"}
          />
          <Form.Item
            label="订单"
            required
            clickable
            onClick={handleSelectOrder}
            extra={selectedOrder?.label || "请选择订单"}
          />
          <Form.Item
            label="工序"
            required
            clickable
            onClick={handleSelectProcess}
            extra={selectedProcess ? `${selectedProcess.name} (¥${selectedProcess.piecePrice})` : (orderId ? "请选择工序" : "请先选择订单")}
          />
          <Form.Item label="数量" required>
            <Stepper min={1} value={quantity} onChange={setQuantity} />
          </Form.Item>
          <Form.Item label="金额">
            <span className="text-lg font-medium text-orange-500">¥{totalAmount}</span>
          </Form.Item>
        </Form>
      </div>
    </div>
  );
}
