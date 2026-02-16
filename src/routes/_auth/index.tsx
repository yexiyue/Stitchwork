import { createFileRoute, redirect } from "@tanstack/react-router";
import { useAuthStore, selectIsBoss, selectIsSuperAdmin } from "@/stores/auth";
import { BossHome } from "./_boss/-home";
import { StaffHome } from "./_staff/-home";

export const Route = createFileRoute("/_auth/")({
  beforeLoad: () => {
    const isSuperAdmin = selectIsSuperAdmin(useAuthStore.getState());
    if (isSuperAdmin) {
      throw redirect({ to: "/admin" });
    }
  },
  component: HomePage,
});

function HomePage() {
  const isBoss = useAuthStore(selectIsBoss);
  return isBoss ? <BossHome /> : <StaffHome />;
}
