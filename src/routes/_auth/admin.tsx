import { createFileRoute, Outlet, redirect } from "@tanstack/react-router";
import { useAuthStore, selectIsSuperAdmin } from "@/stores/auth";

export const Route = createFileRoute("/_auth/admin")({
  beforeLoad: () => {
    const isSuperAdmin = selectIsSuperAdmin(useAuthStore.getState());
    if (!isSuperAdmin) {
      throw redirect({ to: "/" });
    }
  },
  component: () => <Outlet />,
});
