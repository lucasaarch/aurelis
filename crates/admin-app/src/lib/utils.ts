import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export function displayEnum(value: string) {
    return value
        .split("_")
        .map(
            (word) =>
                word.charAt(0).toUpperCase() + word.slice(1).toLowerCase(),
        )
        .join(" ");
}
