import { Component, EventEmitter, Input, Output } from '@angular/core';
import { RessourceInfo } from '../ressource-info';

@Component({
  selector: 'app-ressource-item',
  templateUrl: './ressource-item.component.html',
  styleUrls: ['./ressource-item.component.scss']
})
export class RessourceItemComponent {

  @Input() ressource!: RessourceInfo;
  @Output() newItemEvent = new EventEmitter<string>();

}
