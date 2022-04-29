
<script lang="ts">
    //@ts-ignore
    const invoke = window.__TAURI__.invoke

    import {onDestroy} from 'svelte'
    
    let total_memory = 'Loading...'
    total_memory = invoke('total_memory').then(res=>total_memory=res)

    let used_memory = 'Loading...'
    
    
    // Looping
    const interval = setInterval(async ()=>{

        used_memory = await invoke('used_memory')

    }, 1000);
    onDestroy(() => clearInterval(interval));    

</script>

<p>
    Used Memory: {used_memory}
    <br>
    Total Memory: {total_memory}
</p>