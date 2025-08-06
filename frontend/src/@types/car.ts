import z from 'zod'

const Car = z.object({
  owner: z.string(),
  plate: z.string().optional(),
  brand: z.string().optional(),
  last_revision: z.string().optional(),
  last_road_tax: z.string().optional(),
})

export type Car = z.infer<typeof Car>
export const CarListSchema = z.array(Car)
