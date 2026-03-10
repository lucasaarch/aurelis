import "./app.css";
import { useNavigation } from "@/stores/navigation";
import { VIEWS } from "@/views";

function App() {
  const view = useNavigation((s) => s.view);
  const View = VIEWS[view];

  return <View />;
}

export default App;
