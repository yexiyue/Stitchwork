export interface LoginRequest {
  username: string;
  password: string;
}

export type Role = "boss" | "staff";

export interface LoginUser {
  id: string;
  username: string;
  role: Role;
  displayName?: string;
  phone?: string;
  avatar?: string;
  workshopName?: string;
  workshopDesc?: string;
}

export interface LoginResponse {
  token: string;
  user: LoginUser;
}

export interface RegisterRequest {
  username: string;
  password: string;
}

export interface CreateStaffRequest {
  username: string;
  password: string;
  displayName?: string;
  phone?: string;
}

export interface InviteCodeResponse {
  code: string;
  expiresAt: number;
}

export interface BindBossRequest {
  inviteCode: string;
}

export interface UpdateProfileRequest {
  displayName?: string;
  phone?: string;
  avatar?: string;
  workshopName?: string;
  workshopDesc?: string;
}
