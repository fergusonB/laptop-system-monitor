<script lang="ts">

    //@ts-ignore
    const invoke = window.__TAURI__.invoke;

	let data = {};

	setInterval(async ()=>{
			data = JSON.parse(await invoke("main"))
		},1000)
	
	
	import Memory from './components/Memory.svelte'



</script>

<main>

	<Memory {data}/>

	

</main>

<style>

</style>