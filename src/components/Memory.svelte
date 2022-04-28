
<script lang="ts">
    //@ts-ignore
    const invoke = window.__TAURI__.invoke
        
        invoke('total_memory').then(total_memory => alert(total_memory));

</script>

hello world