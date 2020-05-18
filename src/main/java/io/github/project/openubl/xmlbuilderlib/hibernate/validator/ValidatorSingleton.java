/**
 * Copyright 2019 Project OpenUBL, Inc. and/or its affiliates
 * and other contributors as indicated by the @author tags.
 *
 * Licensed under the Eclipse Public License - v 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * https://www.eclipse.org/legal/epl-2.0/
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package io.github.project.openubl.xmlbuilderlib.hibernate.validator;

import io.github.project.openubl.xmlbuilderlib.clock.SystemClockSingleton;

import javax.validation.Validation;
import javax.validation.Validator;
import javax.validation.ValidatorFactory;

public class ValidatorSingleton {

    private static ValidatorSingleton instance;

    private final ValidatorFactory factory;
    private final Validator validator;

    private ValidatorSingleton() {
        factory = Validation.buildDefaultValidatorFactory();
        validator = factory.getValidator();
    }

    public static ValidatorSingleton getInstance() {
        if (instance == null) {
            synchronized (SystemClockSingleton.class) {
                if (instance == null) {
                    instance = new ValidatorSingleton();
                }
            }
        }
        return instance;
    }

    public ValidatorFactory getFactory() {
        return factory;
    }

    public Validator getValidator() {
        return validator;
    }

}
