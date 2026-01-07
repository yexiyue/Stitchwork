import { createFileRoute, useNavigate } from "@tanstack/react-router";
import {
  Form,
  Input,
  Button,
  NavBar,
  Dialog,
  Toast,
  Checkbox,
  List,
} from "antd-mobile";
import { ChevronLeft } from "lucide-react";
import { useCreateShare, useProcesses, useOrders } from "@/hooks";
import type { CreateShareRequest, Process } from "@/types";
import { useState, useMemo } from "react";

export const Route = createFileRoute("/_auth/_boss/shares/new")({
  component: NewSharePage,
});

function NewSharePage() {
  const navigate = useNavigate();
  const createMutation = useCreateShare();
  const { data: processData, isLoading: processLoading } = useProcesses({
    pageSize: 1000,
  });
  const { data: orderData } = useOrders({ pageSize: 1000 });

  const [selectedProcessIds, setSelectedProcessIds] = useState<string[]>([]);

  const processes = processData?.list ?? [];
  const orders = orderData?.list ?? [];

  // 构建订单ID到产品名的映射
  const orderMap = useMemo(
    () => Object.fromEntries(orders.map((o) => [o.id, o.productName])),
    [orders]
  );

  // 按订单分组工序
  const groupedProcesses = useMemo(() => {
    return processes.reduce(
      (acc, p) => {
        const key = orderMap[p.orderId] || "未分类";
        if (!acc[key]) acc[key] = [];
        acc[key].push(p);
        return acc;
      },
      {} as Record<string, Process[]>
    );
  }, [processes, orderMap]);

  const handleToggleProcess = (id: string) => {
    setSelectedProcessIds((prev) =>
      prev.includes(id) ? prev.filter((x) => x !== id) : [...prev, id]
    );
  };

  const handleSelectAll = (productName: string) => {
    const processIds = groupedProcesses[productName].map((p) => p.id);
    const allSelected = processIds.every((id) =>
      selectedProcessIds.includes(id)
    );
    if (allSelected) {
      setSelectedProcessIds((prev) =>
        prev.filter((id) => !processIds.includes(id))
      );
    } else {
      setSelectedProcessIds((prev) => [
        ...prev.filter((id) => !processIds.includes(id)),
        ...processIds,
      ]);
    }
  };

  const handleSubmit = async (values: { title: string }) => {
    if (selectedProcessIds.length === 0) {
      Toast.show({ content: "请至少选择一个工序" });
      return;
    }

    const data: CreateShareRequest = {
      title: values.title,
      orderIds: [],
      processIds: selectedProcessIds,
    };

    try {
      await createMutation.mutateAsync(data);
      Toast.show({ content: "创建成功" });
      navigate({ to: "/shares" });
    } catch (e) {
      Dialog.alert({
        content: e instanceof Error ? e.message : "创建失败",
        confirmText: "确定",
      });
    }
  };

  return (
    <div className="flex flex-col h-full">
      <NavBar
        onBack={() => navigate({ to: "/shares" })}
        backIcon={<ChevronLeft size={24} />}
      >
        新建招工分享
      </NavBar>

      <div className="flex-1 overflow-auto">
        <Form
          onFinish={handleSubmit}
          footer={
            <Button
              block
              type="submit"
              color="primary"
              loading={createMutation.isPending}
            >
              创建分享
            </Button>
          }
        >
          <Form.Item
            name="title"
            label="分享标题"
            rules={[{ required: true, message: "请输入标题" }]}
          >
            <Input placeholder="如：招工啦" clearable />
          </Form.Item>
        </Form>

        {/* 工序选择 */}
        <div className="px-4 py-2 bg-gray-50">
          <div className="flex justify-between items-center">
            <span className="text-sm text-gray-500">
              选择工序 ({selectedProcessIds.length} 已选)
            </span>
          </div>
        </div>

        {processLoading ? (
          <div className="p-4 text-center text-gray-400">加载中...</div>
        ) : processes.length === 0 ? (
          <div className="p-8 text-center text-gray-400">
            暂无工序，请先在订单中添加工序
          </div>
        ) : (
          Object.entries(groupedProcesses).map(([productName, procs]) => {
            const allSelected = procs.every((p) =>
              selectedProcessIds.includes(p.id)
            );
            const someSelected = procs.some((p) =>
              selectedProcessIds.includes(p.id)
            );

            return (
              <div key={productName} className="mb-2">
                <List
                  header={
                    <div
                      className="flex items-center gap-2 cursor-pointer"
                      onClick={() => handleSelectAll(productName)}
                    >
                      <Checkbox
                        checked={allSelected}
                        indeterminate={someSelected && !allSelected}
                        onChange={() => handleSelectAll(productName)}
                      />
                      <span>{productName}</span>
                    </div>
                  }
                >
                  {procs.map((process) => (
                    <List.Item
                      key={process.id}
                      prefix={
                        <Checkbox
                          checked={selectedProcessIds.includes(process.id)}
                          onChange={() => handleToggleProcess(process.id)}
                        />
                      }
                      description={`¥${process.piecePrice}/件`}
                      onClick={() => handleToggleProcess(process.id)}
                      arrow={false}
                    >
                      {process.name}
                    </List.Item>
                  ))}
                </List>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
}
