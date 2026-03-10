import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { useNavigation } from "@/stores/navigation";
import { VIEWS } from "@/views";
import "./App.css";

const queryClient = new QueryClient();

function App() {
  const view = useNavigation((s) => s.view);
  const View = VIEWS[view];

  return (
    <QueryClientProvider client={queryClient}>
      <div className="app">
        <View />
      </div>
    </QueryClientProvider>
  );
}

export default App;
