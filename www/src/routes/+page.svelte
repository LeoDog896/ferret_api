<script>
    import "../index.css"
    const images = import.meta.glob('./../../../ferret_images/collection/**/image.jpg')
    const metadata = import.meta.glob('./../../../ferret_images/collection/**/image.json')
    
</script>

<ul>
    {#each Object.values(images) as image}
        {@const img = image()}
        {#await img then awaitedImg}
            <li><img src={awaitedImg.default} /></li>
        {/await}
    {/each}
    <li></li>
</ul>