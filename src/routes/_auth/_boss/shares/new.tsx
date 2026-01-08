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
  TextArea,
} from "antd-mobile";
import { ChevronLeft } from "lucide-react";
import { useCreateShare, useProcesses, useOrders } from "@/hooks";
import type { CreateShareRequest, Process, Order } from "@/types";
import { useState, useMemo } from "react";
import { OssImage } from "@/components";

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
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");

  const processes = processData?.list ?? [];
  const orders = orderData?.list ?? [];

  // 构建订单ID到订单的映射
  const orderMap = useMemo(
    () => Object.fromEntries(orders.map((o) => [o.id, o])) as Record<string, Order>,
    [orders]
  );

  // 按订单分组工序
  const groupedProcesses = useMemo(() => {
    return processes.reduce(
      (acc, p) => {
        const order = orderMap[p.orderId];
        const key = order?.productName || "未分类";
        if (!acc[key]) acc[key] = { order, processes: [] };
        acc[key].processes.push(p);
        return acc;
      },
      {} as Record<string, { order?: Order; processes: Process[] }>
    );
  }, [processes, orderMap]);

  const handleToggleProcess = (id: string) => {
    setSelectedProcessIds((prev) =>
      prev.includes(id) ? prev.filter((x) => x !== id) : [...prev, id]
    );
  };

  const handleSelectAll = (productName: string) => {
    const processIds = groupedProcesses[productName].processes.map((p) => p.id);
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

  const handleSubmit = async () => {
    if (!title.trim()) {
      Toast.show({ content: "请输入标题" });
      return;
    }
    if (selectedProcessIds.length === 0) {
      Toast.show({ content: "请至少选择一个工序" });
      return;
    }

    const data: CreateShareRequest = {
      title: title.trim(),
      description: description.trim() || undefined,
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
        right={
          <Button
            size="small"
            color="primary"
            loading={createMutation.isPending}
            onClick={handleSubmit}
          >
            创建
          </Button>
        }
      >
        新建招工分享
      </NavBar>

      <div className="flex-1 overflow-auto">
        <Form>
          <Form.Item label="分享标题">
            <Input
              placeholder="如：招工啦"
              clearable
              value={title}
              onChange={setTitle}
            />
          </Form.Item>
          <Form.Item label="详情说明">
            <TextArea
              placeholder="可选，如：长期招工，待遇优厚"
              rows={2}
              value={description}
              onChange={setDescription}
            />
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
          Object.entries(groupedProcesses).map(([productName, { order, processes: procs }]) => {
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
                    <div className="flex items-center gap-3">
                      <div
                        className="flex items-center gap-2 cursor-pointer flex-1"
                        onClick={() => handleSelectAll(productName)}
                      >
                        <Checkbox
                          checked={allSelected}
                          indeterminate={someSelected && !allSelected}
                          onChange={() => handleSelectAll(productName)}
                        />
                        <span>{productName}</span>
                      </div>
                      {order?.images?.[0] && (
                        <OssImage
                          src={order.images[0]}
                          className="w-10 h-10 rounded object-cover"
                        />
                      )}
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
