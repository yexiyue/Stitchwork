import { Popup } from "antd-mobile";
import type { Order } from "@/types";
import { RelativeTime } from "./RelativeTime";
import { ImageOff } from "lucide-react";
import { Image } from "./OssImage";

interface OrderPickerProps {
  visible: boolean;
  orders: Order[];
  onSelect: (order: Order) => void;
  onClose: () => void;
}

export function OrderPicker({
  visible,
  orders,
  onSelect,
  onClose,
}: OrderPickerProps) {
  return (
    <Popup
      visible={visible}
      onMaskClick={onClose}
      position="bottom"
      showCloseButton
      onClose={onClose}
      bodyStyle={{ borderTopLeftRadius: 8, borderTopRightRadius: 8 }}
    >
      <div className="p-4 text-center font-medium border-b border-gray-200">
        选择订单
      </div>
      <div className="max-h-80 overflow-y-auto p-2">
        {orders.length === 0 ? (
          <div className="text-center text-gray-400 py-8">暂无可选订单</div>
        ) : (
          orders.map((order) => (
            <div
              key={order.id}
              className="flex gap-3 p-3 rounded-lg active:bg-gray-50"
              onClick={() => onSelect(order)}
            >
              {order.images?.length ? (
                <Image
                  src={order.images[0]}
                  width={56}
                  height={56}
                  fit="cover"
                  className="rounded shrink-0"
                />
              ) : (
                <div className="w-14 h-14 bg-gray-100 rounded shrink-0 flex items-center justify-center">
                  <ImageOff size={20} className="text-gray-400" />
                </div>
              )}
              <div className="flex-1 min-w-0 flex flex-col justify-center">
                <span className="font-medium truncate">
                  {order.productName}
                </span>
                <div className="text-sm text-gray-500">
                  数量: {order.quantity}
                </div>
                <RelativeTime date={order.createdAt} />
              </div>
            </div>
          ))
        )}
      </div>
    </Popup>
  );
}
