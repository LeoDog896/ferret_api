import * as files from "../../ferret_images/collection/**";

import { z } from "zod";

const stringKey = z.string().optional().nullable()

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

console.log(files);

const parsedFiles = fileSchema.safeParse(files);

if (parsedFiles.success) {
  for (const [
    key,
    { "image.jpg": image, "image.json": json },
  ] of Object.entries(parsedFiles.data)) {
    const imageElement = document.createElement("img");
    imageElement.src = image;
    document.body.appendChild(imageElement);
  }
} else {
  console.log(JSON.stringify(parsedFiles.error.format(), null, 4));
}
