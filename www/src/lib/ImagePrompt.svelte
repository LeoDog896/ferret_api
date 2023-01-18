<script lang="ts">
	import type { FerretInfo } from './info';

    type License = FerretInfo["info"]["license"];

	export let data: FerretInfo;
	export let img: string;

    interface LicenseData {
        url: string;
        name: string;
    }

    export function parseLicense(license: License) {
        switch(license) {
            case "Attribution":
                return { 
                    url: "https://creativecommons.org/licenses/by/3.0/us/",
                    name: "CC BY"
                }
            case "AttributionShareAlike":
                return { 
                    url: "https://creativecommons.org/licenses/by-sa/3.0/us/",
                    name: "CC BY-SA"
                }
            case "AttributionNoDerivatives":
                return { 
                    url: "https://creativecommons.org/licenses/by-nd/3.0/us/",
                    name: "CC BY-ND"
                }
            case "AttributionNonCommercial":
                return { 
                    url: "https://creativecommons.org/licenses/by-nc/3.0/us/",
                    name: "CC BY-NC"
                }
            case "AttributionNonCommercialShareAlike":
                return { 
                    url: "https://creativecommons.org/licenses/by-nc-sa/3.0/us/",
                    name: "CC BY-NC-SA"
                }
            case "AttributionNonCommercialNoDerivatives":
                return { 
                    url: "https://creativecommons.org/licenses/by-nc-nd/3.0/us/",
                    name: "CC BY-NC-ND"
                }

        }

        throw Error("Unknown license")
    }
</script>

<div id="container">
	<img alt={data.info.alt} src={img} />
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
            {@const parsedLicense = parseLicense(data.license)}
			<p>License: <a href={parsedLicense.url}>{parsedLicense.name}</a></p>
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
