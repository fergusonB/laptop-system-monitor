<script lang="ts">

    export let data


 
</script>

<p>

{#await data}
    Loading...
{:then value}
 {value.free_memory}
{:catch error}
    {error}
{/await}
</p>
