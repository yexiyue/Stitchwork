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
import { useShares, useUpdateShare, useProcesses, useOrders } from "@/hooks";
import type { UpdateShareRequest, Process } from "@/types";
import { useState, useMemo, useEffect } from "react";

export const Route = createFileRoute("/_auth/_boss/shares/$id")({
  component: EditSharePage,
});

function EditSharePage() {
  const { id } = Route.useParams();
  const navigate = useNavigate();
  const { data: shares, isLoading: sharesLoading } = useShares();
  const updateMutation = useUpdateShare();
  const { data: processData, isLoading: processLoading } = useProcesses({
    pageSize: 1000,
  });
  const { data: orderData } = useOrders({ pageSize: 1000 });

  const share = shares?.find((s) => s.id === id);
  const [selectedProcessIds, setSelectedProcessIds] = useState<string[]>([]);
  const [title, setTitle] = useState("");

  // 初始化选中的工序
  useEffect(() => {
    if (share) {
      setSelectedProcessIds(share.processIds);
      setTitle(share.title);
    }
  }, [share]);

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

  const handleToggleProcess = (processId: string) => {
    setSelectedProcessIds((prev) =>
      prev.includes(processId)
        ? prev.filter((x) => x !== processId)
        : [...prev, processId]
    );
  };

  const handleSelectAll = (productName: string) => {
    const processIds = groupedProcesses[productName].map((p) => p.id);
    const allSelected = processIds.every((pid) =>
      selectedProcessIds.includes(pid)
    );
    if (allSelected) {
      setSelectedProcessIds((prev) =>
        prev.filter((pid) => !processIds.includes(pid))
      );
    } else {
      setSelectedProcessIds((prev) => [
        ...prev.filter((pid) => !processIds.includes(pid)),
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

    const data: UpdateShareRequest = {
      title: title.trim(),
      processIds: selectedProcessIds,
    };

    try {
      await updateMutation.mutateAsync({ id, data });
      Toast.show({ content: "保存成功" });
      navigate({ to: "/shares" });
    } catch (e) {
      Dialog.alert({
        content: e instanceof Error ? e.message : "保存失败",
        confirmText: "确定",
      });
    }
  };

  if (sharesLoading) {
    return (
      <div className="flex flex-col h-full">
        <NavBar
          onBack={() => navigate({ to: "/shares" })}
          backIcon={<ChevronLeft size={24} />}
        >
          编辑分享
        </NavBar>
        <div className="flex-1 flex items-center justify-center">加载中...</div>
      </div>
    );
  }

  if (!share) {
    return (
      <div className="flex flex-col h-full">
        <NavBar
          onBack={() => navigate({ to: "/shares" })}
          backIcon={<ChevronLeft size={24} />}
        >
          编辑分享
        </NavBar>
        <div className="flex-1 flex items-center justify-center text-gray-500">
          分享不存在
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full">
      <NavBar
        onBack={() => navigate({ to: "/shares" })}
        backIcon={<ChevronLeft size={24} />}
      >
        编辑分享
      </NavBar>

      <div className="flex-1 overflow-auto">
        <Form
          footer={
            <Button
              block
              color="primary"
              loading={updateMutation.isPending}
              onClick={handleSubmit}
            >
              保存
            </Button>
          }
        >
          <Form.Item label="分享标题">
            <Input
              value={title}
              onChange={setTitle}
              placeholder="如：招工啦"
              clearable
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
