<script lang="ts">
    import "../index.css"

    function assert(condition: boolean, message: string) {
        if (!condition) {
            throw message;
        }
    }

    const data = import.meta.glob('./../../../ferret_images/collection/**/image.{jpg,json}')

    const images = Object.fromEntries(Object.entries(data).filter(([key]) => key.endsWith("jpg")))
    const metadata = Object.fromEntries(Object.entries(data).filter(([key]) => key.endsWith("json")))

    assert(Object.keys(images).length === Object.keys(metadata).length, "Unequal number of images and metadata files")

    const pairs = Object.fromEntries(Object.entries(images).map(([key, value]) => {
        const uuid = key.split("/").slice(-2)[0]
        return [uuid, {image: value, metadata: metadata[`${key.slice(0, -3)}json`]}]
    }))
    
</script>

<ul>
    {#each Object.values(pairs) as pair}
        {@const img = pair.image()}
        {@const meta = pair.metadata()}
        {#await img then awaitedImg}
            {#await meta then awaitedMeta}
                <li><img src={awaitedImg.default} /></li>
            {/await}
        {/await}
    {/each}
    <li></li>
</ul>