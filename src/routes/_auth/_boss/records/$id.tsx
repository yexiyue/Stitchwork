import { createFileRoute, useNavigate } from "@tanstack/react-router";
import {
  NavBar,
  Image,
  Tag,
  Button,
  Dialog,
  Toast,
  Swiper,
  Input,
  Popup,
} from "antd-mobile";
import { ImageIcon, Edit2, Trash2 } from "lucide-react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { pieceRecordApi, orderApi } from "@/api";
import { RelativeTime } from "@/components";
import { RECORD_STATUS_MAP } from "@/constants";
import { useState } from "react";

export const Route = createFileRoute("/_auth/_boss/records/$id")({
  component: RecordDetailPage,
});

function RecordDetailPage() {
  const { id } = Route.useParams();
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const [editPopupVisible, setEditPopupVisible] = useState(false);
  const [editQuantity, setEditQuantity] = useState("");

  const { data: record, isLoading } = useQuery({
    queryKey: ["piece-record", id],
    queryFn: () => pieceRecordApi.getOne(id),
  });

  // 获取订单详情以获取图片列表
  const { data: order } = useQuery({
    queryKey: ["order", record?.orderId],
    queryFn: () => orderApi.getOne(record!.orderId!),
    enabled: !!record?.orderId,
  });

  const approveMutation = useMutation({
    mutationFn: () => pieceRecordApi.approve(id),
    onSuccess: () => {
      Toast.show({ content: "已通过" });
      queryClient.invalidateQueries({ queryKey: ["piece-record", id] });
      queryClient.invalidateQueries({ queryKey: ["piece-records"] });
    },
  });

  const rejectMutation = useMutation({
    mutationFn: () => pieceRecordApi.reject(id),
    onSuccess: () => {
      Toast.show({ content: "已拒绝" });
      queryClient.invalidateQueries({ queryKey: ["piece-record", id] });
      queryClient.invalidateQueries({ queryKey: ["piece-records"] });
    },
  });

  const updateMutation = useMutation({
    mutationFn: (quantity: number) =>
      pieceRecordApi.update(id, { quantity }),
    onSuccess: () => {
      Toast.show({ content: "修改成功" });
      setEditPopupVisible(false);
      queryClient.invalidateQueries({ queryKey: ["piece-record", id] });
      queryClient.invalidateQueries({ queryKey: ["piece-records"] });
    },
  });

  const deleteMutation = useMutation({
    mutationFn: () => pieceRecordApi.delete(id),
    onSuccess: () => {
      Toast.show({ content: "删除成功" });
      queryClient.invalidateQueries({ queryKey: ["piece-records"] });
      navigate({ to: "/records" });
    },
  });

  const handleApprove = () => {
    Dialog.confirm({
      content: "确定通过该计件记录？",
      confirmText: "通过",
      cancelText: "取消",
      onConfirm: () => approveMutation.mutate(),
    });
  };

  const handleReject = () => {
    Dialog.confirm({
      content: "确定拒绝该计件记录？",
      confirmText: "拒绝",
      cancelText: "取消",
      onConfirm: () => rejectMutation.mutate(),
    });
  };

  const handleEdit = () => {
    if (record) {
      setEditQuantity(String(record.quantity));
      setEditPopupVisible(true);
    }
  };

  const handleSaveEdit = () => {
    const qty = parseInt(editQuantity, 10);
    if (isNaN(qty) || qty <= 0) {
      Toast.show({ content: "请输入有效数量" });
      return;
    }
    updateMutation.mutate(qty);
  };

  const handleDelete = () => {
    Dialog.confirm({
      content: "确定删除该计件记录？此操作不可撤销。",
      confirmText: "删除",
      cancelText: "取消",
      onConfirm: () => deleteMutation.mutate(),
    });
  };

  if (isLoading) {
    return (
      <div className="flex flex-col h-full">
        <NavBar onBack={() => navigate({ to: "/records" })}>计件详情</NavBar>
        <div className="flex-1 flex items-center justify-center">加载中...</div>
      </div>
    );
  }

  if (!record) {
    return (
      <div className="flex flex-col h-full">
        <NavBar onBack={() => navigate({ to: "/records" })}>计件详情</NavBar>
        <div className="flex-1 flex items-center justify-center text-gray-500">
          记录不存在
        </div>
      </div>
    );
  }

  const images = order?.images || (record.orderImage ? [record.orderImage] : []);
  const piecePrice = record.piecePrice ? parseFloat(record.piecePrice) : null;

  return (
    <div className="flex flex-col h-full bg-gray-50">
      <NavBar
        onBack={() => navigate({ to: "/records" })}
        right={
          <div className="flex gap-3">
            <Edit2 size={20} onClick={handleEdit} className="text-blue-500" />
            <Trash2 size={20} onClick={handleDelete} className="text-red-500" />
          </div>
        }
      >
        计件详情
      </NavBar>

      <div className="flex-1 overflow-auto">
        {/* 订单图片 */}
        {images.length > 0 ? (
          <div className="bg-white">
            <Swiper>
              {images.map((img, idx) => (
                <Swiper.Item key={idx}>
                  <div className="h-64 flex items-center justify-center bg-gray-100">
                    <Image src={img} fit="contain" height="100%" />
                  </div>
                </Swiper.Item>
              ))}
            </Swiper>
          </div>
        ) : (
          <div className="h-48 bg-white flex items-center justify-center">
            <ImageIcon size={48} className="text-gray-300" />
          </div>
        )}

        {/* 订单信息 */}
        <div className="bg-white mt-2 p-4">
          <div className="text-sm text-gray-500 mb-2">订单信息</div>
          <div className="font-medium text-lg">{record.orderName || "未知订单"}</div>
          {order?.description && (
            <div className="text-sm text-gray-600 mt-1">{order.description}</div>
          )}
          {order?.quantity && (
            <div className="text-sm text-gray-500 mt-1">
              订单数量: {order.quantity}
            </div>
          )}
        </div>

        {/* 工序信息 */}
        <div className="bg-white mt-2 p-4">
          <div className="text-sm text-gray-500 mb-2">工序信息</div>
          <div className="font-medium">{record.processName || "未知工序"}</div>
          {piecePrice !== null && (
            <div className="text-sm text-gray-600 mt-1">
              计件单价: ¥{piecePrice.toFixed(2)}
            </div>
          )}
        </div>

        {/* 计件信息 */}
        <div className="bg-white mt-2 p-4">
          <div className="text-sm text-gray-500 mb-2">计件信息</div>
          <div className="flex justify-between items-center">
            <div>
              <div className="text-2xl font-bold text-blue-600">
                {record.quantity} <span className="text-base font-normal">件</span>
              </div>
              <div className="text-lg text-green-600 mt-1">
                ¥{record.amount}
              </div>
            </div>
            <Tag
              color={RECORD_STATUS_MAP[record.status].color}
              fill="outline"
              style={{ "--border-radius": "4px" }}
            >
              {RECORD_STATUS_MAP[record.status].label}
            </Tag>
          </div>
          <div className="text-sm text-gray-500 mt-3 pt-3 border-t border-gray-100">
            <div className="flex justify-between">
              <span>员工</span>
              <span>{record.userName || "未知员工"}</span>
            </div>
            <div className="flex justify-between mt-2">
              <span>录入方式</span>
              <span>{record.recordedBy === "boss" ? "老板代录" : "员工自报"}</span>
            </div>
            <div className="flex justify-between mt-2">
              <span>录入时间</span>
              <span><RelativeTime date={record.recordedAt} /></span>
            </div>
          </div>
        </div>
      </div>

      {/* 底部操作栏 */}
      {record.status === "pending" && (
        <div className="p-4 bg-white border-t border-gray-100 flex gap-3">
          <Button
            block
            color="danger"
            fill="outline"
            onClick={handleReject}
            loading={rejectMutation.isPending}
          >
            拒绝
          </Button>
          <Button
            block
            color="primary"
            onClick={handleApprove}
            loading={approveMutation.isPending}
          >
            通过
          </Button>
        </div>
      )}

      {/* 编辑弹窗 */}
      <Popup
        visible={editPopupVisible}
        onMaskClick={() => setEditPopupVisible(false)}
        bodyStyle={{ borderTopLeftRadius: 16, borderTopRightRadius: 16 }}
      >
        <div className="p-4">
          <div className="text-lg font-medium mb-4">修改计件数量</div>
          <Input
            type="number"
            value={editQuantity}
            onChange={setEditQuantity}
            placeholder="请输入数量"
            className="border border-gray-200 rounded-lg px-3 py-2"
          />
          {piecePrice !== null && editQuantity && (
            <div className="text-sm text-gray-500 mt-2">
              修改后金额: ¥{(piecePrice * parseInt(editQuantity || "0", 10)).toFixed(2)}
            </div>
          )}
          <div className="flex gap-3 mt-4">
            <Button block fill="outline" onClick={() => setEditPopupVisible(false)}>
              取消
            </Button>
            <Button
              block
              color="primary"
              onClick={handleSaveEdit}
              loading={updateMutation.isPending}
            >
              保存
            </Button>
          </div>
        </div>
      </Popup>
    </div>
  );
}
