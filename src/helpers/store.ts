import {Store} from "tauri-plugin-store-api";
import { invoke } from '@tauri-apps/api/tauri';

export async function getStore(key: string) {
    const path = await invoke('get_store_path');
    const store = new Store(path as string);
    return await store.get(key);
}

export async function setStore(key: string, val: any) {
    const path = await invoke('get_store_path');
    const store = new Store(path as string);
    await store.set(key, val);
    await store.save();
}

export async function deleteStore(key: string) {
    const path = await invoke('get_store_path');
    const store = new Store(path as string);
    await store.delete(key);
    await store.save();
}
