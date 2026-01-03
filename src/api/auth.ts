import { client, setToken } from "./client";
import type {
  LoginRequest,
  LoginResponse,
  RegisterRequest,
  CreateStaffRequest,
  InviteCodeResponse,
  BindBossRequest,
  UpdateProfileRequest,
} from "@/types";

export const authApi = {
  login: async (data: LoginRequest) => {
    const res = await client.post<LoginResponse>("/api/login", data);
    setToken(res.token);
    return res;
  },
  register: (data: RegisterRequest) =>
    client.post<{ userId: string }>("/api/register", data),
  createStaff: (data: CreateStaffRequest) =>
    client.post<{ staffId: string }>("/api/staff", data),
  generateInviteCode: () =>
    client.post<InviteCodeResponse>("/api/invite-code"),
  bindBoss: (data: BindBossRequest) =>
    client.post<void>("/api/bind-boss", data),
  updateProfile: (data: UpdateProfileRequest) =>
    client.put<void>("/api/profile", data),
};
