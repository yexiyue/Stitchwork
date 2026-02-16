import { useMutation } from "@tanstack/react-query";
import { authApi } from "@/api";
import type { LoginRequest, RegisterRequest, UpdateProfileRequest } from "@/types";

export const useLogin = () =>
  useMutation({ mutationFn: (data: LoginRequest) => authApi.login(data) });

export const useRegister = () =>
  useMutation({ mutationFn: (data: RegisterRequest) => authApi.register(data) });

export const useUpdateProfile = () =>
  useMutation({ mutationFn: (data: UpdateProfileRequest) => authApi.updateProfile(data) });
