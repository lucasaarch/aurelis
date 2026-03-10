import type { ComponentType } from "react";
import type { View } from "@/stores/navigation";
import { LoginView } from "./LoginView";

// Registre novas views aqui. O App.tsx cuida do resto.
export const VIEWS: Record<View, ComponentType> = {
  login: LoginView,
  dashboard: () => null, // placeholder
};
