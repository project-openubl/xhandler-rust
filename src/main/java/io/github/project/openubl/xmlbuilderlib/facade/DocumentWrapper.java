package io.github.project.openubl.xmlbuilderlib.facade;

public class DocumentWrapper<T> {

    private final String xml;
    private final T output;

    public DocumentWrapper(String xml, T output) {
        this.xml = xml;
        this.output = output;
    }

    public String getXml() {
        return xml;
    }

    public T getOutput() {
        return output;
    }

}
