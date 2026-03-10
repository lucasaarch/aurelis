import { create } from "zustand";

export type View = "login" | "dashboard";

interface NavigationState {
  view: View;
  navigate: (view: View) => void;
}

export const useNavigation = create<NavigationState>((set) => ({
  view: "login",
  navigate: (view) => set({ view }),
}));
