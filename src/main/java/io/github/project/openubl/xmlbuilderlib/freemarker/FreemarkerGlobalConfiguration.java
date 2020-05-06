/**
 * Copyright 2019 Project OpenUBL, Inc. and/or its affiliates
 * and other contributors as indicated by the @author tags.
 * <p>
 * Licensed under the Eclipse Public License - v 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * <p>
 * https://www.eclipse.org/legal/epl-2.0/
 * <p>
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package io.github.project.openubl.xmlbuilderlib.freemarker;

import freemarker.template.Configuration;

public class FreemarkerGlobalConfiguration {

    private static volatile FreemarkerGlobalConfiguration instance;

    private Configuration configuration;

    private FreemarkerGlobalConfiguration() {
        configuration = new Configuration(Configuration.VERSION_2_3_30);
        configuration.setClassForTemplateLoading(FreemarkerGlobalConfiguration.class, "/");
    }

    public static FreemarkerGlobalConfiguration getInstance() {
        if (instance == null) {
            synchronized (FreemarkerGlobalConfiguration.class) {
                if (instance == null) {
                    instance = new FreemarkerGlobalConfiguration();
                }
            }
        }
        return instance;
    }

    public Configuration getConfiguration() {
        return configuration;
    }

    public void setConfiguration(Configuration configuration) {
        this.configuration = configuration;
    }

}
