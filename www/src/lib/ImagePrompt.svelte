<script lang="ts">
	import type { FerretInfo } from './info';

	export let data: FerretInfo;
    export let img: string;
</script>

<div id="container">
    <img alt={data.info.alt} src={img}>
    <div id="info">
        {#if data.source}
            <p><a href={data.source}>Image Origin</a></p>
        {/if}

        {#if data.author}
            <p>Made by: {data.author}</p>
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

        {#if data.license}
            <p>License: {data.license}</p>
        {:else}
            <p>License: CC0 / Public Domain</p>
        {/if}
    </div>
</div>

<style>
    #container {
        display: flex;
        flex-direction: col;
    }

    img {
        margin-right: 2rem;
        width: 50%;
    }

    p {
        font-family: Verdana;
    }
</style>