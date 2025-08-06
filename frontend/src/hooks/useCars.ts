import { useSuspenseQuery } from "@tanstack/react-query";
import * as R from "ramda";
import { type Car, CarListSchema } from "../@types/car";

const API_BASE_URL = import.meta.env.PUBLIC_API_BASE_URL;

async function fetchCars(): Promise<Car[]> {
	const res = await fetch(R.join("", [API_BASE_URL, "/cars"]));
	if (!res.ok) throw new Error("Failed to fetch cars");
	const json = await res.json();

	return CarListSchema.parse(json);
}

export function useCars() {
	return useSuspenseQuery({
		queryKey: ["cars"],
		queryFn: fetchCars,
	});
}
