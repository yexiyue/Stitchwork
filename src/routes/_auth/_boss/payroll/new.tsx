import { createFileRoute, useNavigate } from "@tanstack/react-router";
import {
  NavBar,
  Toast,
  Form,
  Button,
  Picker,
  Checkbox,
  TextArea,
  Image,
} from "antd-mobile";
import { ChevronLeft, ImageIcon } from "lucide-react";
import type { Staff, PieceRecord } from "@/types";
import { payrollApi, pieceRecordApi } from "@/api";
import { useQueryClient, useMutation, useQuery } from "@tanstack/react-query";
import { useState, useMemo } from "react";
import { useStaffList } from "@/hooks";
import { ImageUploader } from "@/components";

export const Route = createFileRoute("/_auth/_boss/payroll/new")({
  component: NewPayrollPage,
});

function NewPayrollPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const [userId, setUserId] = useState("");
  const [selectedRecordIds, setSelectedRecordIds] = useState<string[]>([]);
  const [paymentImage, setPaymentImage] = useState("");
  const [note, setNote] = useState("");

  const { data: staffList } = useStaffList();

  // Fetch approved records for selected staff
  const { data: recordsData, isLoading: recordsLoading } = useQuery({
    queryKey: ["piece-records-approved", userId],
    queryFn: () =>
      pieceRecordApi.list({
        userId,
        status: ["approved"],
        pageSize: 1000,
      }),
    enabled: !!userId,
  });

  const records = recordsData?.list ?? [];

  const createMutation = useMutation({
    mutationFn: payrollApi.create,
    onSuccess: () => {
      Toast.show({ content: "创建成功" });
      queryClient.invalidateQueries({ queryKey: ["payrolls"] });
      queryClient.invalidateQueries({ queryKey: ["piece-records"] });
      navigate({ to: "/payroll" });
    },
    onError: (err: Error) => {
      Toast.show({ content: err.message || "创建失败" });
    },
  });

  const staffOptions = (staffList?.list ?? []).map((s: Staff) => ({
    label: s.displayName || s.username,
    value: s.id,
  }));

  const selectedStaff = staffOptions.find((o) => o.value === userId);

  // Calculate total amount from selected records
  const totalAmount = useMemo(() => {
    return records
      .filter((r) => selectedRecordIds.includes(r.id))
      .reduce((sum, r) => sum + parseFloat(r.amount), 0)
      .toFixed(2);
  }, [records, selectedRecordIds]);

  const handleSelectStaff = async () => {
    const val = await Picker.prompt({ columns: [staffOptions] });
    if (val) {
      setUserId(val[0] as string);
      setSelectedRecordIds([]);
    }
  };

  const handleToggleRecord = (recordId: string) => {
    setSelectedRecordIds((prev) =>
      prev.includes(recordId)
        ? prev.filter((id) => id !== recordId)
        : [...prev, recordId]
    );
  };

  const handleSelectAll = () => {
    if (selectedRecordIds.length === records.length) {
      setSelectedRecordIds([]);
    } else {
      setSelectedRecordIds(records.map((r) => r.id));
    }
  };

  const handleSubmit = () => {
    if (!userId) {
      Toast.show({ content: "请选择员工" });
      return;
    }
    if (selectedRecordIds.length === 0) {
      Toast.show({ content: "请选择要结算的计件记录" });
      return;
    }
    createMutation.mutate({
      userId,
      amount: totalAmount,
      recordIds: selectedRecordIds,
      paymentImage: paymentImage || undefined,
      note: note || undefined,
    });
  };

  return (
    <div className="flex flex-col h-full bg-gray-50">
      <NavBar
        onBack={() => navigate({ to: "/payroll" })}
        backIcon={<ChevronLeft size={24} />}
      >
        新增工资单
      </NavBar>

      <div className="flex-1 overflow-auto p-4">
        <Form>
          <Form.Item
            label="员工"
            required
            clickable
            onClick={handleSelectStaff}
            extra={selectedStaff?.label || "请选择员工"}
          />
        </Form>

        {userId && (
          <>
            {/* Records selection */}
            <div className="bg-white rounded-lg mt-3 p-3">
              <div className="flex justify-between items-center mb-3">
                <span className="font-medium">待结算计件 ({records.length})</span>
                {records.length > 0 && (
                  <Button size="mini" onClick={handleSelectAll}>
                    {selectedRecordIds.length === records.length
                      ? "取消全选"
                      : "全选"}
                  </Button>
                )}
              </div>

              {recordsLoading ? (
                <div className="text-center text-gray-400 py-6">加载中...</div>
              ) : records.length === 0 ? (
                <div className="text-center text-gray-400 py-6">
                  该员工暂无待结算记录
                </div>
              ) : (
                <div className="space-y-2">
                  {records.map((record: PieceRecord) => (
                    <div
                      key={record.id}
                      className="flex items-center gap-3 p-2 bg-gray-50 rounded-lg"
                      onClick={() => handleToggleRecord(record.id)}
                    >
                      <Checkbox
                        checked={selectedRecordIds.includes(record.id)}
                      />
                      <div className="shrink-0 h-12 aspect-square rounded overflow-hidden bg-gray-200 flex items-center justify-center">
                        {record.orderImage ? (
                          <Image
                            src={record.orderImage}
                            width="100%"
                            height="100%"
                            fit="cover"
                          />
                        ) : (
                          <ImageIcon size={16} className="text-gray-400" />
                        )}
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="text-sm truncate">
                          {record.orderName || "订单"} · {record.processName}
                        </div>
                        <div className="text-xs text-gray-500">
                          数量: {record.quantity}
                        </div>
                      </div>
                      <div className="text-orange-500 font-medium">
                        ¥{record.amount}
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>

            {/* Total amount */}
            <div className="bg-white rounded-lg mt-3 p-4 flex justify-between items-center">
              <span className="text-gray-600">结算金额</span>
              <span className="text-2xl font-bold text-orange-500">
                ¥{totalAmount}
              </span>
            </div>

            {/* Payment image */}
            <div className="bg-white rounded-lg mt-3 p-3">
              <div className="text-gray-600 mb-2">支付凭证（可选）</div>
              <ImageUploader
                value={paymentImage}
                onChange={setPaymentImage}
                maxCount={1}
              />
            </div>

            {/* Note */}
            <div className="bg-white rounded-lg mt-3 p-3">
              <div className="text-gray-600 mb-2">备注（可选）</div>
              <TextArea
                value={note}
                onChange={setNote}
                placeholder="添加备注..."
                rows={2}
              />
            </div>
          </>
        )}
      </div>

      {/* Submit button */}
      <div className="p-4 bg-white border-t border-gray-200">
        <Button
          block
          color="primary"
          onClick={handleSubmit}
          loading={createMutation.isPending}
          disabled={!userId || selectedRecordIds.length === 0}
        >
          确认发放 ¥{totalAmount}
        </Button>
      </div>
    </div>
  );
}
