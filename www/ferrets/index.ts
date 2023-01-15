import * as files from "../../ferret_images/collection/**";

import { z } from "zod";

const stringKey = z.string().optional().nullable();

const container = document.querySelector("ul")!;

// keyed object -- UUID as key, object as value with image.png and image.json as keys, and string for image.png and object for image.json
const fileSchema = z.record(
  z.string(),
  z.object({
    "image.jpg": z.string(),
    "image.json": z.object({
      info: z.object({
        name: stringKey,
        sex: stringKey,
        color: stringKey,
        pattern: stringKey,
        alt: stringKey,
      }),
      author: stringKey,
      source: stringKey,
      license: stringKey,
    }),
  })
);

const parsedFiles = fileSchema.safeParse(files);

console.log(parsedFiles);

if (parsedFiles.success) {
  document.title = `Ferrets (${Object.keys(parsedFiles.data).length})`
  for (const [
    key,
    { "image.jpg": image, "image.json": json },
  ] of Object.entries(parsedFiles.data)) {
    const imageElement = document.createElement("img");
    imageElement.src = image;

    const li = document.createElement("li");
    li.appendChild(imageElement);
    container.appendChild(li);
  }

  const li = document.createElement("li");
  container.appendChild(li);
} else {
  console.log(JSON.stringify(parsedFiles.error.format(), null, 4));
}
