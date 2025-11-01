import init, * as wasm from './wasm/promify.js';

const menuItemId = "promify";

chrome.runtime.onInstalled.addListener(async () => {
    try {
        await init();
    } catch (err) {
        console.error('Failed to init WASM module:', err);
    }
    chrome.contextMenus.create({
        id: menuItemId,
        title: "Promify",
        contexts: ["selection"],
    });
});

chrome.contextMenus.onClicked.addListener((info, tab) => {
    if (info.menuItemId === menuItemId && info.selectionText) {
        const newText = wasm.promify(info.selectionText);
        chrome.scripting.executeScript({
            target: { tabId: tab.id },
            args: [newText],
            function: copyTextToClipboard,
        });
    }
});

async function copyTextToClipboard(text) {
    try {
        await navigator.clipboard.writeText(text);
    } catch (err) {
        console.error('Failed to copy text using Clipboard API:', err);
    }
}
