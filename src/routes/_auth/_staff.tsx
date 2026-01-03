import { createFileRoute, Outlet, redirect } from "@tanstack/react-router";
import { useAuthStore, selectIsBoss } from "@/stores/auth";

export const Route = createFileRoute("/_auth/_staff")({
  beforeLoad: () => {
    const isBoss = selectIsBoss(useAuthStore.getState());
    if (isBoss) {
      throw redirect({ to: "/" });
    }
  },
  component: () => <Outlet />,
});
