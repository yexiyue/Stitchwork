import { createFileRoute } from "@tanstack/react-router";
import { RegisterCodeManager } from "@/components";

export const Route = createFileRoute("/_auth/admin/register-codes")({
  component: RegisterCodesPage,
});

function RegisterCodesPage() {
  return <RegisterCodeManager queryKey={["admin", "register-codes"]} />;
}
