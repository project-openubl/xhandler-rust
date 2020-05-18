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
