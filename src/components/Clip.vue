<template>
    <div>
        <h1 class="text-3xl font-bold text-amber-300 underline">
            Hello world!
        </h1>
        <h2>📋 Clipboard History</h2>
        <ul class="grid grid-cols-2 gap-5">
            <li class="rounded-md border-slate-200 border p-10" v-for="(item, index) in clipboardHistory" :key="index">
                {{ item }}
            </li>
        </ul>
    </div>
</template>

<script lang="ts" setup>
import { listen } from "@tauri-apps/api/event";

import { ref, onMounted } from "vue";
const clipboardHistory = ref<string[]>([])


onMounted(() => {
    listen<string>('clipboard-update', (event) => {
        const text = event.payload;

        // prevent the duplicates text
        if (!clipboardHistory.value.includes(text)) {
            clipboardHistory.value.unshift(text)

            if (clipboardHistory.value.length > 20) {
                clipboardHistory.value.pop()
            }
        }
    })
})

</script>