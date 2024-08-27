import SearchResult from './SearchResult';
import { useStore } from './reactStore';

export default function ResultList() {
  const { queryResult } = useStore();
  return (
    <ul className="result-container grow">
      {queryResult.results.map((item, index) => (
        <SearchResult
          key={index + item.heading + item.subheading}
          index={index}
          {...item}
        />
      ))}
    </ul>
  );
}
