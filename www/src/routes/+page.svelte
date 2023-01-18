<script lang="ts">
	import { onMount } from 'svelte';
	import '../index.css';
	import { ferretInfo, type FerretInfo } from '$lib/info';
	import { getContext } from 'svelte';
	import ImagePrompt from '$lib/ImagePrompt.svelte';

	const { open } = getContext('simple-modal') as any;
	const showInfo = (data: FerretInfo, img: string) => open(ImagePrompt, { data, img });

	function assert(condition: boolean, message: string) {
		if (!condition) {
			throw message;
		}
	}

	const data = import.meta.glob('./../../../ferret_images/collection/**/image.{jpg,json}');

	const images = Object.fromEntries(Object.entries(data).filter(([key]) => key.endsWith('jpg')));
	const metadata = Object.fromEntries(Object.entries(data).filter(([key]) => key.endsWith('json')));

	assert(
		Object.keys(images).length === Object.keys(metadata).length,
		'Unequal number of images and metadata files'
	);

	const pairs = Object.fromEntries(
		Object.entries(images).map(([key, value]) => {
			const uuid = key.split('/').slice(-2)[0];
			return [uuid, { image: value, metadata: metadata[`${key.slice(0, -3)}json`] }];
		})
	);

	onMount(() => {
		document.title = `Ferret Images (${Object.keys(pairs).length})`;
	});
</script>

<ul>
	{#each Object.values(pairs) as pair}
		{@const img = pair.image()}
		{@const meta = pair.metadata()}
		{#await img then awaitedImg}
			{#await meta then awaitedMeta}
				{@const info = ferretInfo.parse(awaitedMeta)}
				<li>
					<img
						alt={info.info.alt}
						on:keypress={(event) => {
							if (event.key === 'Enter') {
								showInfo(info);
							}
						}}
						on:click={() => showInfo(info, awaitedImg.default)}
						src={awaitedImg.default}
					/>
				</li>
			{/await}
		{/await}
	{/each}
	<li />
</ul>

<style lang="scss">
	ul {
		display: flex;
		flex-wrap: wrap;
		list-style: none;
		margin: 0.5rem;
		padding: 0;
	}

	li {
		flex-grow: 1;
		padding: 0.5rem;
	}

	img {
		max-height: 100%;
		min-width: 100%;
		height: 10rem;
		object-fit: cover;
		vertical-align: bottom;

		transition: all 0.2s;

		// Hover indication
		&:hover {
			transform: scale(1.05);
			box-shadow: 0 0 0.5rem 0.5rem rgba(0, 0, 0, 0.1);
			cursor: pointer;
		}
	}

	li:last-child {
		flex-grow: 10;
	}
</style>
