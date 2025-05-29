import React, { createContext, useContext, useEffect, useState, useCallback } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { tryCatch } from "@/lib/try-catch";
import { toast } from "sonner";

interface WindowContextType {
    isWindowMaximized: boolean;
    minimizeWindow: () => Promise<void>;
    toggleMaximizeWindow: () => Promise<void>;
    closeWindow: () => Promise<void>;
    maximizeWindow: () => Promise<void>;
    unmaximizeWindow: () => Promise<void>;
}

const WindowContext = createContext<WindowContextType | undefined>(undefined);

export const WindowProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const [isWindowMaximized, setIsWindowMaximized] = useState(false);
    const appWindow = getCurrentWindow();

    const updateIsWindowMaximized = useCallback(async () => {
        const isMaximizedResult = await tryCatch(appWindow.isMaximized());
        if (isMaximizedResult.error) {
            toast.error("Failed to check if the window is maximized.");
            return;
        }
        setIsWindowMaximized(isMaximizedResult.data);
    }, [appWindow]);

    const minimizeWindow = async () => {
        if (appWindow) {
            const actionResult = await tryCatch(appWindow.minimize());
            if (actionResult.error) {
                toast.error("Failed to minimize the window.");
            }
        }
    };

    const toggleMaximizeWindow = async () => {
        if (appWindow) {
            const actionResult = await tryCatch(appWindow.toggleMaximize());
            if (actionResult.error) {
                toast.error("Failed to minimize the window.");
            }
        }
    };

    const maximizeWindow = async () => {
        if (appWindow) {
            const actionResult = await tryCatch(appWindow.maximize());
            if (actionResult.error) {
                toast.error("Failed to minimize the window.");
            }
        }
    };

    const unmaximizeWindow = async () => {
        if (appWindow) {
            const actionResult = await tryCatch(appWindow.unmaximize());
            if (actionResult.error) {
                toast.error("Failed to minimize the window.");
            }
        }
    };

    const closeWindow = async () => {
        if (appWindow) {
            const actionResult = await tryCatch(appWindow.close());
            if (actionResult.error) {
                toast.error("Failed to minimize the window.");
            }
        }
    };

    useEffect(() => {
        updateIsWindowMaximized();

        let unlisten: () => void = () => {};
        const listen = async () => {
            unlisten = await appWindow.onResized(() => {
                updateIsWindowMaximized();
            });
        };

        listen();
        return () => unlisten?.();
    }, [appWindow, updateIsWindowMaximized]);

    return (
        <WindowContext.Provider
            value={{
                isWindowMaximized,
                minimizeWindow,
                toggleMaximizeWindow,
                closeWindow,
                maximizeWindow,
                unmaximizeWindow,
            }}>
            {children}
        </WindowContext.Provider>
    );
};

export const useWindow = () => {
    const context = useContext(WindowContext);
    if (!context) {
        throw new Error("useWindow must be used within a WindowProvider");
    }
    return context;
};
