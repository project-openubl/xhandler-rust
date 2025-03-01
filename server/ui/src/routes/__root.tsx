import React, { Suspense } from "react";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Outlet, createRootRoute } from "@tanstack/react-router";

import { NotificationsProvider } from "@app/components/NotificationsProvider.tsx";
import { DefaultLayout } from "@app/layout";

import "@patternfly/patternfly/patternfly-addons.css";
import "@patternfly/patternfly/patternfly.css";

const TanStackQueryDevtools =
	process.env.NODE_ENV === "production"
		? () => null
		: React.lazy(() =>
				import("@tanstack/react-query-devtools").then((res) => ({
					default: res.ReactQueryDevtools,
				})),
			);

const TanStackRouterDevtools =
	process.env.NODE_ENV === "production"
		? () => null
		: React.lazy(() =>
				import("@tanstack/router-devtools").then((res) => ({
					default: res.TanStackRouterDevtools,
				})),
			);

const queryClient = new QueryClient();

export const Route = createRootRoute({
	component: RootComponent,
});

function RootComponent() {
	return (
		<>
			<QueryClientProvider client={queryClient}>
				<NotificationsProvider>
					<DefaultLayout>
						<Outlet />
					</DefaultLayout>
				</NotificationsProvider>
				<Suspense>
					<TanStackQueryDevtools position="bottom" initialIsOpen={false} />
				</Suspense>
				<Suspense>
					<TanStackRouterDevtools position="bottom-left" />
				</Suspense>
			</QueryClientProvider>
		</>
	);
}
