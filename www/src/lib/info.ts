import zod from 'zod';

const optionalNullableString = zod.string().nullable().optional();

export const ferretInfo = zod.object({
	info: zod.object({
		name: optionalNullableString,
		sex: optionalNullableString,
		color: optionalNullableString,
		pattern: optionalNullableString,
		alt: optionalNullableString
	}),
	author: optionalNullableString,
	source: optionalNullableString,
	license: zod
		.enum([
			'Attribution',
			'AttributionShareAlike',
			'AttributionNoDerivatives',
			'AttributionNonCommercial',
			'AttributionNonCommercialShareAlike',
			'AttributionNonCommercialNoDerivatives'
		])
		.nullable()
		.optional()
});

export type FerretInfo = zod.infer<typeof ferretInfo>;
