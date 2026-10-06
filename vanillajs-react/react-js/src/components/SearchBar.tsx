import { useState } from 'react';

export function SearchBar({ loading, onSearch }: { loading: boolean; onSearch: (city: string) => void }) {
  const [value, setValue] = useState('');

  function search() {
    const city = value.trim();
    if (city && !loading) onSearch(city);
  }

  return (
    <section className="weather__search">
      <input
        type="text"
        placeholder="Enter city"
        className="weather__input js-weather-input"
        value={value}
        onChange={(event) => setValue(event.target.value)}
        onKeyDown={(event) => event.key === 'Enter' && search()}
      />
      <button className="weather__button js-weather-search-button" type="button" disabled={loading} onClick={search}>
        {loading ? 'Loading...' : 'Search'}
      </button>
    </section>
  );
}
