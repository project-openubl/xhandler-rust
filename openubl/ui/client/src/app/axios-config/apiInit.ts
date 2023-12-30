import axios from "axios";

export const initInterceptors = (access_token: string) => {
  axios.interceptors.request.use(
    (config) => {
      const token = access_token;
      if (token) {
        config.headers.Authorization = `Bearer ${token}`;
      }
      return config;
    },
    (error) => {
      return Promise.reject(error);
    }
  );

  // axios.interceptors.response.use(
  //   (response) => {
  //     return response;
  //   },
  //   async (error) => {
  //     if (error.response && error.response.status === 401) {
  //       try {
  //         const refreshed = await keycloak.updateToken(5);
  //         if (refreshed) {
  //           const retryConfig = {
  //             ...error.config,
  //             headers: {
  //               ...error.config.headers,
  //               Authorization: `Bearer ${keycloak.token}`,
  //             },
  //           };
  //           return axios(retryConfig);
  //         }
  //       } catch (refreshError) {
  //         keycloak.login();
  //       }
  //     }
  //     return Promise.reject(error);
  //   }
  // );
};
