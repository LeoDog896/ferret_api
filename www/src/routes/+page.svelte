<script lang="ts">
	import { onMount } from 'svelte';
	import PhGithubLogoDuotone from '~icons/ph/github-logo-duotone';
	import PhSuitcaseDuotone from '~icons/ph/suitcase-duotone';
	import '@fontsource/fira-mono';

	let data: any = {};
	let imageURL = '';

	onMount(async () => {
		data = await fetch('https://ferrets.leodog896.com/v1/data/random').then((response) =>
			response.json()
		);

		imageURL = `https://ferrets.leodog896.com${data.url}`;
	});
</script>

<div>
	<main>
		<h1>Ferret API</h1>
		<div class="buttons">
			<a href="https://github.com/LeoDog896/ferret_api">
				<PhGithubLogoDuotone height="2rem" width="2rem" />
			</a>
			<a href="/ferret_api/business">
				<PhSuitcaseDuotone height="2rem" width="2rem" />
			</a>
		</div>
		<p>A <span class="hot">RESTful API</span> for <span class="hot">Ferret</span> images.</p>

		<p>
			Want to contribute your own images? Head over to the <code class="bubble"
				><a href="https://github.com/LeoDog896/ferret_images#readme">ferret_images</a></code
			> repository!
		</p>

		<div class="displayBlock">
			<div class="title"><h2>Example Image</h2></div>
			<div class="imgContainer">
				<img src={imageURL} alt="Ferret" />
			</div>
		</div>

		<div class="displayBlock cold">
			<div class="title cold"><h2>Example Data</h2></div>
			<code class="code black">
				<pre>{JSON.stringify(data, null, 4)}</pre>
			</code>
		</div>

		<div class="displayBlock">
			<div class="title"><h2>Documentation</h2></div>
			<div class="code left-margin">
				<p>
					<b>GET</b>
					<a href="https://ferrets.leodog896.com/v1/image/random">
						https://ferrets.leodog896.com/v1/image/random
					</a>
					<br />
					<span class="description">returns an affixed URL to a ferret .JPG</span>
				</p>
				<p>
					<b>GET</b>
					<a href="https://ferrets.leodog896.com/v1/data/random">
						https://ferrets.leodog896.com/v1/data/random
					</a>
					<br />
					<span class="description">returns an affixed JSON object to a ferret.json file</span>
				</p>
				<p>
					<b>GET</b> https://ferrets.leodog896.com/v1/image/uuid/{'{uuid}'}
					<br />
					<span class="description">returns a .JPG of a ferret for some UUID.</span>
				</p>
				<p>
					<b>GET</b> https://ferrets.leodog896.com/v1/data/uuid/{'{uuid}'}
					<br />
					<span class="description"
						>returns the JSON object of a ferret.json file for some UUID.</span
					>
				</p>
			</div>
		</div>

		<p>
			(Want a business of ferrets? Check out the <a href="/ferret_api/business">business dump</a>!)
		</p>
	</main>
</div>

<style lang="scss">
	div {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	h1 {
		margin-bottom: 0;
	}

	.left-margin {
		margin-left: 1rem;
	}

	code.bubble {
		background-color: var(--purple);
		border-radius: 0.5rem;
		padding: 0.5rem;

		a {
			color: white;
		}
	}

	main {
		margin: 1rem;
		text-align: center;
		display: flex;
		flex-direction: column;
		max-width: 800px;
	}

	a {
		color: var(--cold);
	}

	img {
		border-bottom: 0;
		max-width: calc(800px - 0.5rem);
		width: auto;
		height: auto;
		max-height: 800px;
	}

	.imgContainer {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		background-image: radial-gradient(var(--purple) 1px, transparent 0);
		background-size: 5px 5px;
		background-position: -2px -2px;
	}

	.code {
		display: flex;
		flex-direction: column;
		align-items: start;
		justify-content: start;
		font-family: monospace;

		p {
			text-align: left;

			.description {
				color: slategray;
				margin-left: 1rem;
			}
		}

		pre {
			text-align: left;
			max-width: calc(800px - 2.5rem);
			overflow-x: scroll;
			margin: 1rem;
			padding-bottom: 1rem;
		}

		&.black {
			color: white;
			background-color: black;
		}

		* {
			font-family: 'Fira Mono', monospace;
		}
	}

	div.title {
		background-color: var(--hot);
		width: 100%;
		color: white;

		&.cold {
			background-color: var(--cold);
		}
	}

	.displayBlock {
		border-radius: 1rem 1rem 0 0;
		border: 0.25rem solid var(--hot);
		display: flex;
		flex-direction: column;
		align-items: start;
		margin-bottom: 1rem;

		&.cold {
			border: 0.25rem solid var(--cold);
		}
	}
</style>
