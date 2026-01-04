import { client } from "./client";
import type { HomeOverview, ActivityList } from "@/types";

export const homeApi = {
  overview: () => client.get<HomeOverview>("/api/home/overview"),
  activities: () => client.get<ActivityList>("/api/home/activities"),
};
