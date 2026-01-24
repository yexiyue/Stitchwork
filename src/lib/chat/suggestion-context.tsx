import {
  createContext,
  useContext,
  useState,
  useCallback,
  type ReactNode,
} from "react";

export interface Suggestion {
  prompt: string;
}

interface SuggestionContextValue {
  suggestions: Suggestion[];
  isLoading: boolean;
  setSuggestions: (suggestions: Suggestion[]) => void;
  addSuggestion: (suggestion: Suggestion) => void;
  clearSuggestions: () => void;
  setIsLoading: (loading: boolean) => void;
}

const SuggestionContext = createContext<SuggestionContextValue | null>(null);

export function SuggestionProvider({ children }: { children: ReactNode }) {
  const [suggestions, setSuggestionsState] = useState<Suggestion[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  const setSuggestions = useCallback((newSuggestions: Suggestion[]) => {
    setSuggestionsState(newSuggestions);
  }, []);

  const addSuggestion = useCallback((suggestion: Suggestion) => {
    setSuggestionsState((prev) => [...prev, suggestion]);
  }, []);

  const clearSuggestions = useCallback(() => {
    setSuggestionsState([]);
  }, []);

  return (
    <SuggestionContext.Provider
      value={{
        suggestions,
        isLoading,
        setSuggestions,
        addSuggestion,
        clearSuggestions,
        setIsLoading,
      }}
    >
      {children}
    </SuggestionContext.Provider>
  );
}

export function useSuggestions() {
  const context = useContext(SuggestionContext);
  if (!context) {
    throw new Error("useSuggestions must be used within a SuggestionProvider");
  }
  return context;
}
