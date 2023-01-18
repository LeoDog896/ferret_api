<script lang="ts">
	import type { FerretInfo } from './info';

	export let data: FerretInfo;
</script>

{#if data.source}
    <p>Source: <a href={data.source}>{data.source}</a></p>
{/if}

{#if data.author}
    <p>Made by {data.author}</p>
{/if}

{#if data.info.name}
    <p>Ferret's Name: {data.info.name}</p>
{/if}

{#if data.info.sex}
    <p>Ferret's Sex: {data.info.sex}</p>
{/if}

{#if data.info.color}
    <p>Ferret's Color: {data.info.color}</p>
{/if}

{#if data.info.pattern}
    <p>Ferret's Pattern: {data.info.pattern}</p>
{/if}

<style>
    p {
        font-family: Roboto;
    }
</style>