import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { useState } from 'react';
import { CurrentWeather } from './components/CurrentWeather';
import { ErrorBanner } from './components/ErrorBanner';
import { Forecast } from './components/Forecast';
import { SearchBar } from './components/SearchBar';
import { ThemeToggle } from './components/ThemeToggle';
import { type Search, useCurrentWeather, useForecast } from './hooks/queries';
import { useTheme } from './hooks/useTheme';
import './styles/style.css';
import './styles/theme.css';
import './styles/weather.css';
import './styles/root.css';

export function createQueryClient() {
  // The original fetched once per click, with no retries or background refetching.
  return new QueryClient({
    defaultOptions: { queries: { retry: false, refetchOnWindowFocus: false, staleTime: Infinity } },
  });
}

function WeatherApp() {
  const { theme, toggle } = useTheme();
  const [search, setSearch] = useState<Search | null>(null);
  const [forecastOpen, setForecastOpen] = useState(false);
  const weather = useCurrentWeather(search);
  const forecast = useForecast(search, forecastOpen);
  const error = weather.error ?? forecast.error;

  function onSearch(city: string) {
    // A new search starts with the forecast closed; its forecast belongs to this city only.
    setForecastOpen(false);
    setSearch((previous) => ({ id: (previous?.id ?? 0) + 1, city }));
  }

  function onShowForecast() {
    setForecastOpen(true);
    if (forecast.isError) forecast.refetch();
  }

  const showForecast = forecastOpen && forecast.isSuccess;

  return (
    <>
      <ThemeToggle theme={theme} onToggle={toggle} />

      <header>
        <h1 className="weather__title">Weather App</h1>
      </header>

      <main>
        <SearchBar loading={weather.isFetching} onSearch={onSearch} />

        {error && <ErrorBanner error={error} />}

        {weather.isSuccess && !weather.isFetching && (
          <CurrentWeather
            weather={weather.data}
            forecastOpen={showForecast}
            forecastLoading={forecast.isFetching}
            onShowForecast={onShowForecast}
          />
        )}

        {showForecast && <Forecast data={forecast.data} onHide={() => setForecastOpen(false)} />}
      </main>

      <footer className="weather__footer">
        <p>Built with React</p>
      </footer>
    </>
  );
}

export function App({ queryClient }: { queryClient: QueryClient }) {
  return (
    <QueryClientProvider client={queryClient}>
      <WeatherApp />
    </QueryClientProvider>
  );
}
