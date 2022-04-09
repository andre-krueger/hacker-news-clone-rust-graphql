import AsyncStorage from "@react-native-async-storage/async-storage";

type Key = "cookie";

export function StorageService(
  errorHandler: (error: Error) => void
): { getItem: typeof getItem; setItem: typeof setItem } {
  async function getItem(key: Key): Promise<string | null | void> {
    try {
      return await AsyncStorage.getItem(key);
    } catch (error) {
      errorHandler(error);
    }
  }

  async function setItem(key: Key, value: string) {
    try {
      await AsyncStorage.setItem(key, value);
    } catch (error) {
      errorHandler(error);
    }
  }

  return { getItem, setItem };
}
