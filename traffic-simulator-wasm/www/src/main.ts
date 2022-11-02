import "./main.css";
import App from "./App.svelte";

const app = new App({
    target: document.body,
    props: {
        itemCode: import.meta.env.VITE_ITEM_CODE || '<no item code found>'
    }
});

export default app;
