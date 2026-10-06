import { useQuery } from '@tanstack/react-query';
import { getWeatherByCity, getWeatherForecastByCity } from '../api/weather';

/** One submitted search. `id` makes searching the same city again fetch again, as the original did. */
export interface Search {
  id: number;
  city: string;
}

export function useCurrentWeather(search: Search | null) {
  return useQuery({
    queryKey: ['weather', search?.id, search?.city],
    queryFn: () => getWeatherByCity(search!.city),
    enabled: search !== null,
  });
}

export function useForecast(search: Search | null, enabled: boolean) {
  return useQuery({
    queryKey: ['forecast', search?.id, search?.city],
    queryFn: () => getWeatherForecastByCity(search!.city),
    enabled: search !== null && enabled,
  });
}
