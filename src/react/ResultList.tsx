import SearchResult from './SearchResult';
import { useStore } from './reactStore';

export default function ResultList() {
  const { queryResult } = useStore();
  return (
    <ul className="result-container">
      {queryResult.results.map((item, index) => (
        <SearchResult
          key={index + item.heading + item.subheading}
          index={index}
          iconPath={item.iconPath}
          heading={item.heading}
          subtext={item.subheading}
        />
      ))}
    </ul>
  );
}
