<template>
    <h3>Ini history gan</h3>
    <ul class="grid grid-cols-2 gap-5">
        <li class="rounded-md border-slate-200 border p-4" v-for="(item, index) in clipboardHistory" :key="index">
            <p class="text-sm font-mono">{{ item.content }}</p>
            <p class="text-xs text-gray-500 mt-1">{{ item.timestamp }}</p>
        </li>
    </ul>
</template>


<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
// import { invoke } from "@tauri-apps/api/tauri";
import { ref, onMounted } from "vue";
interface ClipboardEntry {
    content: string;
    timestamp: string;
}



const clipboardHistory = ref<ClipboardEntry[]>([])

console.log(clipboardHistory.value)
onMounted(async () => {
    // Load existing history on mount
    try {
        const result = await invoke<ClipboardEntry[]>("get_clipboard_history");
        clipboardHistory.value = result.reverse(); // Show newest first
    } catch (e) {
        console.error("Failed to load clipboard history:", e);
    }

    // Listen for live updates
    listen<string>('clipboard-update', (event) => {
        const text = event.payload;

        // prevent duplicates
        if (!clipboardHistory.value.some(entry => entry.content === text)) {
            clipboardHistory.value.unshift({
                content: text,
                timestamp: new Date().toISOString(),
            });

            if (clipboardHistory.value.length > 20) {
                clipboardHistory.value.pop();
            }
        }
    });
});
</script>
