import { standardSchemaResolver } from "@hookform/resolvers/standard-schema";
import { Shield } from "lucide-react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { LoginForm } from "@/components/login-form";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

const loginSchema = z.object({
  email: z.email("E-mail inválido."),
  password: z.string().min(1, "Senha obrigatória."),
});

export type LoginValues = z.infer<typeof loginSchema>;

export function LoginView() {
  const form = useForm<LoginValues>({
    resolver: standardSchemaResolver(loginSchema),
    defaultValues: { email: "", password: "" },
  });

  async function onSubmit(_data: LoginValues) {
    // TODO: integrar com a API
    // const { user, token } = await api.login(data);
    // login(user, token);
    // navigate("dashboard");
  }

  return (
    <div className="flex min-h-screen items-center justify-center bg-background px-4">
      <div className="w-full max-w-sm space-y-6">
        <div className="flex flex-col items-center gap-2 text-center">
          <div className="flex size-12 items-center justify-center rounded-xl bg-primary text-primary-foreground">
            <Shield className="size-6" />
          </div>
          <h1 className="text-2xl font-semibold tracking-tight">Aurelis</h1>
          <p className="text-sm text-muted-foreground">
            Painel de administração
          </p>
        </div>

        <Card>
          <CardHeader className="pb-4">
            <CardTitle className="text-base">Entrar</CardTitle>
            <CardDescription>
              Digite suas credenciais para acessar o painel.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <LoginForm form={form} onSubmit={onSubmit} />
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
